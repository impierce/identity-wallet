import java.util.Properties
import java.nio.file.Files

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

val keystoreProperties = Properties()
val keystorePropertiesFile = rootProject.file("key.properties").toPath()
if (Files.exists(keystorePropertiesFile)) {
    Files.newBufferedReader(keystorePropertiesFile).use { reader ->
        keystoreProperties.load(reader)
    }
}

android {
    compileSdk = 33
    namespace = "com.impierce.identity_wallet"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false"
        applicationId = "com.impierce.identity_wallet"
        minSdk = 24
        targetSdk = 33
        versionCode = 15
        versionName = "0.2.4"
    }
    signingConfigs {
        create("release") {
            storeFile = file(keystoreProperties["storeFile"])
            storePassword = keystoreProperties["storePassword"] as String
            keyAlias = keystoreProperties["keyAlias"] as String
            keyPassword = keystoreProperties["keyPassword"] as String
        }
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            isMinifyEnabled = true
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
            signingConfig = signingConfigs.getByName("release")
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
}

rust {
    rootDirRel = "../../../"
}

dependencies {
    implementation("androidx.webkit:webkit:1.6.1")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.8.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")
