# Backup retention

UniMe keeps **three backups**: the most recent one, one from roughly a week ago,
and one from roughly a fortnight ago. Everything else is deleted.

Implemented in [`identity-wallet/src/state/backup/retention.rs`](../identity-wallet/src/state/backup/retention.rs).

## When backups are created

Two things create a backup, and they are independent of each other:

| Trigger                                     | When                                                       |
| ------------------------------------------- | ---------------------------------------------------------- |
| **Back up automatically** (settings switch) | On unlock, if the newest backup is more than 24 hours old. |
| **Back up now** (button)                    | Immediately, whenever the user taps it.                    |

The switch is only a preference. Turning it on does not create a backup, and
turning it off does not delete one — it just stops new backups being taken on
unlock. Existing backups stay, and "Back up now" keeps working either way.

Automatic backups only happen at unlock because that is the one moment the
profile password is in memory: archives are encrypted directly from it with
Argon2id, so nothing can be sealed without the user having just typed it.

## The policy

Retention runs immediately after a backup is written, which is the only moment
the store grows. Three slots are filled, and any backup in none of them is
deleted:

| Slot        | Holds                                           |
| ----------- | ----------------------------------------------- |
| **latest**  | The newest backup.                              |
| **ageing**  | The _oldest_ backup still younger than 14 days. |
| **archive** | The newest backup that is at least 14 days old. |

Slots can land on the same backup, so a young profile keeps fewer than three.
The maximum is three.

### Why "ageing" is the oldest recent backup, not the newest

The obvious reading of "keep one from last week and one from two weeks ago" is to
sort backups into age buckets of 0–7, 7–14 and 14+ days and keep the newest in
each. That does not work.

With a backup a day, yesterday's backup is in the same 0–7 bucket as today's, and
loses to it. It is deleted. So is the one before it. No backup ever survives long
enough to reach the 7-day bucket, and the store collapses to a single backup
forever:

```text
day  1: kept ages [0]
day  2: kept ages [0]
day  8: kept ages [0]
day 24: kept ages [0]
```

Filling the older slots means deliberately keeping a backup that is _neither_ the
newest _nor_ old enough to matter yet, purely so that it can age into them. That
is the ageing slot. Because a backup leaves that slot by turning into an archive,
the archive slot rotates too, instead of pinning the first backup forever.

## Worked example

A backup every day, showing the ages of what is kept:

```text
day  1: kept ages [0]
day  2: kept ages [0, 1]
day  3: kept ages [0, 2]
day  8: kept ages [0, 7]
day 14: kept ages [0, 13]
day 15: kept ages [0, 1, 14]
day 21: kept ages [0, 7, 20]
day 28: kept ages [0, 1, 14]
day 35: kept ages [0, 8, 21]
day 42: kept ages [0, 2, 15]
```

In steady state the three ages cycle over a fortnight: roughly 0 days, 0–13 days
and 14–27 days.

## Consequences worth knowing

- **The ageing slot drifts.** It is not pinned at exactly seven days; it sweeps
  from 0 to 13 days and then becomes the archive. Holding a backup at a fixed age
  would need a fourth slot.
- **Manual backups are pruned by the same rule.** Tapping "Back up now" twice in
  one day leaves one of them: both are younger than 14 days, so the newer takes
  the latest slot and the older takes the ageing slot, and a third tap evicts the
  middle one. There is no "pin this backup" concept.
- **A profile left alone for months keeps one backup.** Nothing is recent enough
  to separate the slots, so they collapse onto the newest. The next unlock adds a
  fresh one.
- **Backups with an unreadable timestamp are never deleted.** The store writes
  these timestamps itself, so an unparseable one means something is already wrong,
  and pruning should not compound it by deleting what it cannot reason about.

## Changing the policy

`ARCHIVE_AGE` in `retention.rs` is the single knob: it sets both when a backup
becomes an archive and, therefore, how long the ageing slot sweeps for. The
policy is a pure function of a backup listing and the current time
(`retention::expired`), so a change can be checked against the unit tests in that
file without touching the store.
