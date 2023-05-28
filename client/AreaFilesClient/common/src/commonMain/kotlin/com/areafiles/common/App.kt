package com.areafiles.common

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

@Composable
fun App() {
    var text by remember { mutableStateOf("Hello, area files client!") }
    val platformName = getPlatformName()

    Column(modifier = Modifier.fillMaxWidth()) {
        Button(
            onClick = {
                text = "Hello, area files client on $platformName"
            },
            modifier = Modifier.align(Alignment.CenterHorizontally)
        ) {
            Text(text)
        }
    }
}
