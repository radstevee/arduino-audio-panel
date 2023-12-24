package net.radstevee.goxlr.actions

object AudioUtil {
    fun setAppVolume(app: String, volume: Double) {
        // SEND FUCKING HELP THIS TOOK TOO LONG TO DEBUG
        val processBuilder = ProcessBuilder(
            "bash", "-c",
            """
            set_volume() {
                SINK_INPUT_INDEX=${'$'}(pactl list sink-inputs | awk -v stream="$app" '${'$'}1 == "Sink" && ${'$'}2 == "Input" { sink_input_index = ${'$'}3 } ${'$'}0 ~ "application.name = \\\"" stream "\\\"" { print substr(sink_input_index, 2); exit }')
            
                if [ -n "${"\$SINK_INPUT_INDEX"}" ]; then
                    echo "${'$'}1"
                    pactl set-sink-input-volume "${"\$SINK_INPUT_INDEX"}" "${'$'}1"
                fi
            }
    
            set_volume $volume
            """
        )

        try {
            val process = processBuilder.start()
            process.waitFor()
            val exitCode = process.exitValue()
            if (exitCode != 0) {
                println("Error: Failed to set volume. Exit code: $exitCode")
            }
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}