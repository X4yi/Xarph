import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Item {
    id: workspaceWidget
    height: 40

    WorkspaceModelBridge {
        id: workspaceModel
        Component.onCompleted: loadWorkspaces()
    }

    RowLayout {
        anchors.fill: parent
        spacing: 4

        Repeater {
            model: workspaceModel.workspaceCount

            Rectangle {
                width: 32
                height: 28
                radius: 4
                color: index === workspaceModel.focusedIdx ? "#8c6eff" : (wsMouseArea.containsMouse ? "#2a2a4e" : "transparent")
                border.color: index === workspaceModel.focusedIdx ? "#8c6eff" : "#3a3a4e"
                border.width: 1

                property bool isHovered: wsMouseArea.containsMouse

                Text {
                    anchors.centerIn: parent
                    text: index + 1
                    font.pixelSize: 12
                    font.bold: index === workspaceModel.focusedIdx
                    color: index === workspaceModel.focusedIdx ? "#ffffff" : "#a0a0b0"
                }

                MouseArea {
                    id: wsMouseArea
                    anchors.fill: parent
                    hoverEnabled: true
                    acceptedButtons: Qt.LeftButton | Qt.RightButton
                    onClicked: function(mouse) {
                        if (mouse.button === Qt.LeftButton) {
                            workspaceModel.focusWorkspace(index)
                        } else if (mouse.button === Qt.RightButton) {
                            contextMenu.buildMenu("workspace", "Workspace " + (index + 1))
                            contextMenu.x = mouse.x
                            contextMenu.y = mouse.y
                            contextMenu.visible = true
                        }
                    }
                }
            }
        }
    }

    Timer {
        interval: 5000
        running: true
        repeat: true
        onTriggered: workspaceModel.loadWorkspaces()
    }
}
