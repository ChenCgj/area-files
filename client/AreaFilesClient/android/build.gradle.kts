plugins {
    id("org.jetbrains.compose")
    id("com.android.application")
    kotlin("android")
}

group "com.areafiles"
version "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":common"))
    implementation("androidx.activity:activity-compose:1.4.0")
}

android {
    compileSdk = 33
    defaultConfig {
        applicationId = "com.areafiles.android"
//        minSdkVersion(24)
//        targetSdkVersion(33)
        minSdk = 24
        targetSdk = 33
        versionCode = 1
        versionName = "1.0-SNAPSHOT"
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
        }
    }
}