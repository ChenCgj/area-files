package com.areafiles.common

import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class User {
    var type: String = ""
    var ip: String = ""
    var hostname: String = ""
    var uname: String = ""
    var uid: String = ""

    constructor(jsonElement: JsonElement) {
        if (jsonElement.jsonObject["UserLAN"] != null) {
            hostname = jsonElement.jsonObject["UserLAN"]?.jsonObject?.get("host_name")?.jsonPrimitive?.content!!
            ip = jsonElement.jsonObject["UserLAN"]?.jsonObject?.get("ip")?.jsonPrimitive?.content!!
        } else {
            uname = jsonElement.jsonObject["UserWAN"]?.jsonObject?.get("name")?.jsonPrimitive.toString()
            uid = jsonElement.jsonObject["UserWAN"]?.jsonObject?.get("uid")?.jsonPrimitive.toString()
        }
    }
}
