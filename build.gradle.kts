plugins {
    kotlin("jvm") version "1.9.21"
    application
}

group = "net.radstevee"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    testImplementation("org.jetbrains.kotlin:kotlin-test")
    implementation("com.fazecast:jSerialComm:2.0.0")
}

kotlin {
    jvmToolchain(17)
}

application {
    mainClass.set("net.radstevee.goxlr.Main")
}