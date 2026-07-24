package com.impierce.identity_wallet

import android.net.Uri
import android.os.Bundle
import androidx.annotation.Keep
import androidx.browser.customtabs.CustomTabsIntent

@Keep
class MainActivity : TauriActivity() {
    companion object {
        init {
            System.loadLibrary("unime")
        }
    }

    @Keep
    fun openCustomTab(url: String) {
        runOnUiThread {
            val customTabsIntent = CustomTabsIntent.Builder()
                .setShowTitle(true)
                .build()
            customTabsIntent.launchUrl(this, Uri.parse(url))
        }
    }

    private external fun java_init(context: android.content.Context): Boolean

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        java_init(this)
    }

    override fun onNewIntent(intent: android.content.Intent) {
        super.onNewIntent(intent)
        setIntent(intent)
    }
}
