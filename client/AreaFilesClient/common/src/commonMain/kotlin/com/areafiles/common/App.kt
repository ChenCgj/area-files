package com.areafiles.common

import androidx.compose.foundation.ScrollState
import androidx.compose.material.Text
import androidx.compose.material.Button
import androidx.compose.material.IconButton
import androidx.compose.material.Card
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Icon
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.*
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.dp
import java.net.Socket
import kotlinx.serialization.json.*

@Composable
fun App() {
    var info by remember { mutableStateOf(listOf<FileInfo>()) }

    Column(modifier = Modifier.fillMaxWidth()) {
        Title()
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
            Text("update files")
        }
        println(info.size)
        val scrollState = ScrollState(0)
        Column(
            modifier = Modifier
                .align(Alignment.CenterHorizontally)
                .fillMaxWidth(0.8F)
                .verticalScroll(scrollState)
        ) {
            info.forEach {
                FileInfoItem(it)
                Spacer(modifier = Modifier.height(10.dp))
            }
        }
    }
}

@Composable
fun Title() {
    Box(
        modifier = Modifier
            .fillMaxWidth()
            .background(color = Color.hsl(209.06f, 0.9102f, 0.5196f))
    ) {
        Row {
            IconButton(
                onClick = {},
                modifier = Modifier.align(Alignment.CenterVertically)
            ) {
                Icon(imageVector = Icons.Rounded.Menu, contentDescription = "menu")
            }
            Text(
                text = "Area Files",
                modifier = Modifier.align(Alignment.CenterVertically),
                color = Color.Yellow,
                fontSize = 2.em
            )
        }
    }
}

@Composable
fun FileInfoItem(info: FileInfo) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .background(color = Color.hsl(209.06f, 0.9102f, 0.5196f), shape = RoundedCornerShape(8.dp))
            .padding(10.dp)
    ) {
        Column(modifier = Modifier.background(color = Color.hsl(209.06f, 0.9102f, 0.5196f))) {
            Row {
                Text(
                    text = info.path.split("/").last(),
                    modifier = Modifier.align(Alignment.CenterVertically),
                    color = Color.Yellow,
                    fontSize = 1.2.em
                )
                IconButton(
                    modifier = Modifier.align(Alignment.CenterVertically),
                    onClick = {
                        val socket = Socket("localhost", 11114)
                        val outStream = socket.getOutputStream()
                        outStream.write("CMD download ${info.user?.ip} ${info.path}\n".toByteArray())
                        socket.close()
                    }
                ) {
                    Icon(imageVector = Icons.Rounded.AddCircle, contentDescription = "")
                }
            }
            Row() {
                Text("user:")
                Text(info.user!!.ip)
            }
            Spacer(modifier = Modifier.height(5.dp))
            Row() {
                Text("size:")
                Text(info.size.toString())
            }
            Spacer(modifier = Modifier.height(5.dp))
            Row() {
                Text("path:")
                Text(info.path)
            }
        }
    }
}
