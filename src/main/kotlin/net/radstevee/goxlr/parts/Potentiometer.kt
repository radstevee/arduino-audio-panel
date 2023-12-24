package net.radstevee.goxlr.parts

import net.radstevee.goxlr.Main
import kotlin.math.abs

class Potentiometer(private val id: Int, private val onChange: (Potentiometer) -> Unit) {
    @Volatile
    var value: Int = 0
        private set
    val percentage: Int
        get() = synchronized(this) {
            (value / 1023.0 * 100).toInt()
        }

    private var processingThread = Thread {
        while (true) {
            try {
                val line = Main.serialReader.readLine() ?: continue
                if (!line.startsWith("pot")) continue

                val parts = line.split("=")
                if (parts.size == 2 && parts[0].length <= 4) {
                    val potId = parts[0].substring(3).toInt()
                    val potValue = parts[1].toInt()

                    if (potId == id && abs(potValue - value) >= 5) {
                        synchronized(this) {
                            value = potValue
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