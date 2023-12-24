package net.radstevee.goxlr.core

import com.fazecast.jSerialComm.SerialPort
import java.io.BufferedReader
import java.io.InputStreamReader

class SerialReader(portName: String) {
    private val serialPort = SerialPort.getCommPort(portName)
    init {
        serialPort.openPort()
        serialPort.setComPortTimeouts(SerialPort.TIMEOUT_READ_SEMI_BLOCKING, 0, 0)
    }

    private val reader = BufferedReader(InputStreamReader(serialPort.inputStream))
    fun readLine(): String? {
        return reader.readLine()
    }
}
