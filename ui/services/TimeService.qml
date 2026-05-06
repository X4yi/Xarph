import QtQuick 2.15
import QtQuick.Window 2.15

Item {
    id: root

    property date currentDate: new Date()
    property string formattedTime: root.formatTime(root.currentDate)

    property string timeFormat: "24h"
    property string customFormat: "hh:mm:ss"

    Timer {
        id: timeTimer
        interval: 1000
        repeat: true
        running: true
        onTriggered: {
            root.currentDate = new Date()
        }
    }

    function formatTime(date) {
        if (root.timeFormat === "12h") {
            var hours = date.getHours()
            var minutes = date.getMinutes()
            var ampm = hours >= 12 ? "PM" : "AM"
            hours = hours % 12
            hours = hours ? hours : 12
            return (hours < 10 ? "0" : "") + hours + ":" +
                   (minutes < 10 ? "0" : "") + minutes + " " + ampm
        } else if (root.timeFormat === "24h") {
            var h = date.getHours()
            var m = date.getMinutes()
            return (h < 10 ? "0" : "") + h + ":" +
                   (m < 10 ? "0" : "") + m
        } else {
            return Qt.formatDateTime(date, root.customFormat)
        }
    }

    Component.onCompleted: {
        console.log("TimeService initialized")
    }
}
