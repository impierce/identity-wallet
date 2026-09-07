package com.impierce.identity_wallet

import android.os.Bundle

class MainActivity : TauriActivity() {
    companion object {
        init {
            System.loadLibrary("unime")
        }
    }

    private external fun java_init(context: android.content.Context): Boolean

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        java_init(this)
    }
}
