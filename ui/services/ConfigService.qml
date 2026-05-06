import QtQuick 2.15
import Qt.labs.settings 1.0

Item {
    id: root

    readonly property string configPath: StandardPaths.writableLocation(StandardPaths.ConfigLocation) + "/x4-shell/shell/settings.json"

    property var config: ({})
    property var sidebarConfig: root.config.sidebar || ({})
    property var clockConfig: root.config.clock || ({})

    property string sidebarPosition: root.sidebarConfig.position || "left"
    property string clockFormat: root.clockConfig.format || "24h"
    property string clockCustomFormat: root.clockConfig.customFormat || "hh:mm:ss"

    function loadConfig() {
        var xhr = new XMLHttpRequest()
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                if (xhr.status === 200) {
                    try {
                        root.config = JSON.parse(xhr.responseText)
                        console.log("Config loaded:", JSON.stringify(root.config))
                    } catch(e) {
                        console.error("Failed to parse config:", e)
                    }
                } else {
                    console.log("Config file not found, using defaults")
                    root.config = root.defaultConfig()
                }
            }
        }
        xhr.open("GET", "file://" + root.configPath)
        xhr.send()
    }

    function defaultConfig() {
        return {
            "sidebar": {
                "position": "left"
            },
            "clock": {
                "format": "24h"
            }
        }
    }

    Component.onCompleted: {
        root.loadConfig()
        console.log("ConfigService initialized")
    }
}
