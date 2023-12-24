package net.radstevee.goxlr.parts

import net.radstevee.goxlr.Main

fun Int.toBoolean() = this != 0
fun Boolean.toInt() = if (this) 1 else 0

class Button(private val id: Int, private val onChange: (Button) -> Unit) {
    @Volatile
    var value: Boolean = false
        private set

    private var processingThread = Thread {
        while (true) {
            try {
                val line = Main.serialReader.readLine() ?: continue
                if (!line.startsWith("but")) continue

                val parts = line.split("=")
                if (parts.size == 2 && parts[0].length <= 4) {
                    val butId = parts[0].substring(3).toInt()
                    val butValue = parts[1].toInt()

                    if (butId == id && butValue != value.toInt()) {
                        synchronized(this) {
                            value = butValue.toBoolean()
                        }
                        onChange(this)
                    }
                }
            } catch (e: InterruptedException) {
                e.printStackTrace()
                cancel()
                break
            }
        }
    }

    fun start() {
        processingThread.start()
    }

    fun cancel() {
        processingThread.interrupt()
    }
}