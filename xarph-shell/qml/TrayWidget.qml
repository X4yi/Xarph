import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Item {
    id: trayWidget
    height: 40

    TrayModelBridge {
        id: trayModel
        Component.onCompleted: refresh()
    }

    RowLayout {
        anchors.fill: parent
        spacing: 4

        Repeater {
            model: trayModel.itemCount

            Rectangle {
                Layout.fillHeight: true
                width: 28
                height: 28
                radius: 4
                color: trayHoverArea.containsMouse ? "#2a2a4e" : "transparent"

                property string itemId: ""
                property string iconName: ""
                property string title: ""

                Text {
                    anchors.centerIn: parent
                    text: iconName || "●"
                    font.pixelSize: 14
                    color: "#c0c0d0"
                }

                MouseArea {
                    id: trayHoverArea
                    anchors.fill: parent
                    hoverEnabled: true
                    acceptedButtons: Qt.LeftButton | Qt.RightButton
                    onClicked: function(mouse) {
                        if (mouse.button === Qt.LeftButton) {
                            // TODO: activate tray item
                        } else if (mouse.button === Qt.RightButton) {
                            // Show tray context menu
                        }
                    }
                }

                ToolTip {
                    visible: trayHoverArea.containsMouse
                    text: title
                }
            }
        }
    }

    // Refresh tray items periodically
    Timer {
        interval: 3000
        running: true
        repeat: true
        onTriggered: trayModel.refresh()
    }
}
