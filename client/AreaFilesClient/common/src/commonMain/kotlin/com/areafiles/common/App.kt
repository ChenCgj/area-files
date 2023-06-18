package com.areafiles.common

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.Text
import androidx.compose.material.Button
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import java.net.Socket
import kotlinx.serialization.json.*

@Composable
fun App() {
    var info by remember { mutableStateOf(listOf<FileInfo>()) }

    Column(modifier = Modifier.fillMaxWidth()) {
        Button(
            onClick = {
                val socket = Socket("localhost", 11114)
                val inStream = socket.getInputStream()
                val outStream = socket.getOutputStream()
                outStream.write("CMD list my\n".toByteArray())
                val ret = inStream.readNBytes(16)
                val size = ret.toString(Charsets.UTF_8).toBigInteger()
                val jsonBuf = inStream.readNBytes(size.toInt())
                val jsonElement = Json.parseToJsonElement(jsonBuf.toString(Charsets.UTF_8))
                if (jsonElement.jsonObject["statu"]?.jsonPrimitive.toString().toInt() != 0) {
                    println("error: ${jsonElement.jsonObject["msg"]?.jsonPrimitive}")
                }
                val temp = mutableListOf<FileInfo>()
                jsonElement.jsonObject["info"]?.jsonArray?.forEach {
                    val fileinfo = FileInfo(it)
                    temp.add(fileinfo)
                }
                info = temp
                socket.close()
            },
            modifier = Modifier.align(Alignment.CenterHorizontally)
        ) {
            Text("show file information")
        }
        println(info.size)
        info.forEach {
            Box(modifier = Modifier.align(Alignment.CenterHorizontally)) {
                Button(onClick = {
                    val socket = Socket("localhost", 11114)
                    val inStream = socket.getInputStream()
                    val outStream = socket.getOutputStream()
                    outStream.write("CMD download ${it.user?.ip} ${it.path}\n".toByteArray())
                    socket.close()
                }) {
                    Text("download ${it.path}")
                }
            }
        }
    }
}
