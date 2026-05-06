import QtQuick 2.15
import QtQuick.Layouts 1.15
import qs.services 1.0 as Services

ColumnLayout {
    id: root

    spacing: 8
    Layout.fillWidth: true

    property var workspaceService: Services.WorkspaceService

    Repeater {
        model: root.workspaceService ? root.workspaceService.workspaces : []

        delegate: Rectangle {
            id: wsItem

            Layout.alignment: Qt.AlignHCenter
            width: 40
            height: 40
            radius: 8

            color: modelData.focused ? "#4a90d9" : "#2a2a3e"
            border.color: modelData.focused ? "#6ab0ff" : "#3a3a4e"
            border.width: 1

            Text {
                anchors.centerIn: parent
                text: modelData.name || modelData.id
                color: modelData.focused ? "white" : "#888888"
                font.pixelSize: 12
                font.bold: modelData.focused
            }

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    if (root.workspaceService && !modelData.focused) {
                        root.workspaceService.switchWorkspace(modelData.id)
                    }
                }
            }
        }
    }
}
