# UniFFI & JNA keep rules
-keep class uniffi.** { *; }
-keep class com.sun.jna.** { *; }
-dontwarn com.sun.jna.**

# kotlinx.serialization
-keepattributes *Annotation*, InnerClasses
-dontnote kotlinx.serialization.SerializationKt
-keepclassmembers class * {
    @kotlinx.serialization.SerialName <fields>;
}
-keepclassmembers class * {
    *** Companion;
}
-keepclasseswithmembers class * {
    kotlinx.serialization.KSerializer serializer(...);
}
-keep,allowobfuscation,allowshrinking class * {
    <init>(...);
}

# Fortune Note Data Models
-keep class com.fortunexample.note.data.** { *; }
-keepclassmembers class com.fortunexample.note.data.** { *; }
