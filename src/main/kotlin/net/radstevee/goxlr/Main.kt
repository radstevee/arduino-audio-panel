package net.radstevee.goxlr

import net.radstevee.goxlr.actions.AudioUtil
import net.radstevee.goxlr.core.SerialReader
import net.radstevee.goxlr.parts.Button
import net.radstevee.goxlr.parts.Potentiometer
import net.radstevee.goxlr.parts.toInt

class Main {
    companion object {
        val serialReader = SerialReader("/dev/ttyUSB0")

        @JvmStatic
        fun main(args: Array<String>) {
            Main()
        }
    }

    init {
        var multiplier = 1
        val pots = listOf(
            Potentiometer(0) {
                AudioUtil.setAppVolume("spotify", multiplier * it.percentage / 100.0)
            },
            Potentiometer(1) {
                println("2: ${it.percentage}%")
            },
            Potentiometer(2) {
                println("3: ${it.percentage}%")
            }
        )
        pots.forEach { it.start() }
        val button = Button(0) {
            multiplier = it.value.toInt()
            AudioUtil.setAppVolume("spotify", multiplier * pots.first().percentage / 100.0)
        }
        button.start()
    }
}
