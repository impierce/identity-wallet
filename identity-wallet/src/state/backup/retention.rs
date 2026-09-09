//! Which backups are kept, and which are removed to make room.
//!
//! Three backups are kept: the latest, one roughly a week old, and one roughly a
//! fortnight old. Anything else is deleted once a new backup is written.
//!
//! The obvious way to express that — sort backups into age buckets of `0-7`,
//! `7-14` and `14+` days and keep the newest in each — does not work. With a
//! backup a day, yesterday's backup sits in the same `0-7` bucket as today's and
//! loses to it, so no backup ever survives long enough to reach the second
//! bucket, and the store collapses to a single backup forever.
//!
//! Filling the older slots means deliberately keeping a backup that is neither
//! the newest nor old enough to matter yet, purely so that it can age into them.
//! That is the [`ageing`](Slot::Ageing) slot below, and it is what makes the
//! policy work:
//!
//! | Slot | Holds |
//! | --- | --- |
//! | latest | The newest backup. |
//! | ageing | The *oldest* backup still younger than [`ARCHIVE_AGE`] — the one closest to graduating. |
//! | archive | The newest backup that has reached [`ARCHIVE_AGE`]. |
//!
//! A backup leaves the ageing slot by turning into an archive, which is what lets
//! the archive slot rotate instead of pinning the first backup forever. In steady
//! state the three ages cycle over a fortnight, roughly `0`, `0-13` and `14-27`
//! days.

use chrono::{DateTime, TimeDelta, Utc};
use log::info;

use crate::{
    error::AppError,
    state::backup::{
        backup_store,
        store::{BackupFile, BackupStore},
    },
};

/// How old a backup has to be before it counts as an archive rather than a
/// recent one.
const ARCHIVE_AGE: TimeDelta = TimeDelta::days(14);

/// Ids of the backups that fall outside the policy and should be deleted.
///
/// `now` is a parameter rather than read here so the policy can be tested at
/// fixed points in time.
///
/// A backup whose timestamp cannot be read is never returned. Deleting something
/// we cannot reason about is the one outcome worth avoiding, and the store
/// generates these timestamps itself, so an unreadable one means something is
/// wrong that pruning should not compound.
pub fn expired(backups: &[BackupFile], now: DateTime<Utc>) -> Vec<String> {
    // The store's listings are unordered by contract, so impose an order here
    // rather than relying on one.
    let mut dated: Vec<(&BackupFile, DateTime<Utc>)> = backups
        .iter()
        .filter_map(|backup| {
            DateTime::parse_from_rfc3339(&backup.modified_at)
                .ok()
                .map(|written| (backup, written.with_timezone(&Utc)))
        })
        .collect();
    dated.sort_by(|left, right| right.1.cmp(&left.1));

    let is_archive = |written: DateTime<Utc>| now.signed_duration_since(written) >= ARCHIVE_AGE;

    let latest = dated.first();
    // Oldest of the ones that are not archives yet: `dated` is newest first, so
    // this is the last of them.
    let ageing = dated.iter().filter(|(_, written)| !is_archive(*written)).last();
    let archive = dated.iter().find(|(_, written)| is_archive(*written));

    let kept: Vec<&str> = [latest, ageing, archive]
        .into_iter()
        .flatten()
        .map(|(backup, _)| backup.id.as_str())
        .collect();

    dated
        .iter()
        .map(|(backup, _)| backup.id.as_str())
        .filter(|id| !kept.contains(id))
        .map(str::to_string)
        .collect()
}

/// Applies the policy to the backup store.
///
/// Called after a backup is written, which is the only moment the store grows.
pub fn prune() -> Result<(), AppError> {
    let store = backup_store();
    for id in expired(&store.list()?, Utc::now()) {
        info!("deleting backup `{id}`: outside the retention policy");
        store.delete(&id)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-09-09T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    /// A backup written `age_in_days` ago. Only the timestamp and id matter here.
    fn aged(age_in_days: i64) -> BackupFile {
        BackupFile {
            id: format!("age-{age_in_days}"),
            name: "unime.unime".to_string(),
            size: 1,
            modified_at: (now() - TimeDelta::days(age_in_days)).to_rfc3339(),
        }
    }

    /// Ages of the backups the policy keeps, oldest last.
    fn kept(backups: &[BackupFile]) -> Vec<String> {
        let expired = expired(backups, now());
        let mut kept: Vec<String> = backups
            .iter()
            .map(|backup| backup.id.clone())
            .filter(|id| !expired.contains(id))
            .collect();
        kept.sort();
        kept
    }

    #[test]
    fn a_single_backup_is_kept() {
        assert!(expired(&[aged(0)], now()).is_empty());
    }

    #[test]
    fn nothing_is_deleted_when_there_is_nothing_to_delete() {
        assert!(expired(&[], now()).is_empty());
    }

    #[test]
    fn the_newest_and_the_oldest_recent_one_are_kept() {
        // The middle backup is redundant: it is neither the latest nor the one
        // closest to becoming an archive.
        assert_eq!(kept(&[aged(0), aged(3), aged(6)]), ["age-0", "age-6"]);
    }

    #[test]
    fn yesterdays_backup_survives_so_that_it_can_age() {
        // The whole point of the ageing slot. Under a plain "newest per bucket"
        // policy this would be deleted and no backup would ever reach a week old.
        assert!(expired(&[aged(0), aged(1)], now()).is_empty());
    }

    #[test]
    fn all_three_slots_fill_once_a_backup_has_aged_out() {
        assert_eq!(
            kept(&[aged(0), aged(1), aged(7), aged(14), aged(20)]),
            ["age-0", "age-14", "age-7"]
        );
    }

    #[test]
    fn at_most_three_backups_are_ever_kept() {
        let backups: Vec<BackupFile> = (0..40).map(aged).collect();
        let kept = 40 - expired(&backups, now()).len();
        assert_eq!(kept, 3);
    }

    #[test]
    fn the_archive_slot_holds_the_newest_archive_rather_than_the_first_one() {
        // Keeping the *oldest* archive instead would pin the very first backup
        // forever and the slot would never rotate.
        assert_eq!(kept(&[aged(0), aged(14), aged(30), aged(90)]), ["age-0", "age-14"]);
    }

    #[test]
    fn a_profile_left_alone_for_months_keeps_only_the_newest() {
        // Nothing is recent enough for the latest or ageing slots to differ, so
        // the three slots collapse onto one backup rather than keeping stale ones.
        assert_eq!(kept(&[aged(60), aged(75), aged(90)]), ["age-60"]);
    }

    #[test]
    fn an_unreadable_timestamp_is_never_deleted() {
        let mut broken = aged(30);
        broken.modified_at = "not a timestamp".to_string();
        let expired = expired(&[aged(0), aged(1), broken.clone()], now());
        assert!(!expired.contains(&broken.id));
    }

    /// Applies the policy day by day, the way it actually runs: prune straight
    /// after each new backup, against whatever survived the previous rounds.
    ///
    /// This is the property the naive "newest per age bucket" policy fails. It
    /// looks correct on a fixed listing and still collapses to one backup here,
    /// because nothing survives long enough to reach the older buckets.
    #[test]
    fn daily_backups_settle_into_three() {
        let start = now();
        let mut store: Vec<BackupFile> = Vec::new();

        for day in 0..60 {
            let today = start + TimeDelta::days(day);
            store.push(BackupFile {
                id: format!("day-{day}"),
                name: "unime.unime".to_string(),
                size: 1,
                modified_at: today.to_rfc3339(),
            });

            let expired = expired(&store, today);
            store.retain(|backup| !expired.contains(&backup.id));

            assert!(store.len() <= 3, "day {day} kept {} backups", store.len());
            // Once a backup has had time to age all the way through, all three
            // slots stay occupied rather than collapsing onto the newest.
            if day >= 15 {
                assert_eq!(store.len(), 3, "day {day} kept {} backups", store.len());
            }
        }
    }

    #[test]
    fn an_unordered_listing_is_handled() {
        // Listings are unordered by contract, so the result must not depend on
        // the order the store happened to return.
        let ordered = kept(&[aged(0), aged(5), aged(20)]);
        let shuffled = kept(&[aged(20), aged(0), aged(5)]);
        assert_eq!(ordered, shuffled);
    }
}
