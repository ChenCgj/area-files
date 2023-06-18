package com.areafiles.common

import java.math.BigInteger
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class FileInfo(
    var path: String,
    var time: BigInteger,
    var size: BigInteger,
    var tokens: List<Token>?,
    var user: User?) {

    constructor(jsonvalue: JsonElement) : this("", BigInteger.valueOf(0), BigInteger.valueOf(0), null, null) {
        path = jsonvalue.jsonObject["path"]?.jsonPrimitive?.content!!
        time = jsonvalue.jsonObject["time"]?.jsonPrimitive.toString().toBigInteger()
        size = jsonvalue.jsonObject["size"]?.jsonPrimitive.toString().toBigInteger()
        jsonvalue.jsonObject["tokens"]?.jsonArray?.forEach {
            // to do
        }
        user = jsonvalue.jsonObject["user"]?.let { User(it) }
    }
}