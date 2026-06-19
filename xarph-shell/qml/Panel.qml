import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Rectangle {
    id: panel
    height: 40
    color: "#0c0c12"
    border.color: "#1a1a2e"
    border.width: 1

    PanelBridge {
        id: panelBridge
        Component.onCompleted: loadPanelConfig()
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: 8
        anchors.rightMargin: 8
        spacing: 4

        RowLayout {
            spacing: 4
            StartButton {
                id: startButton
                Layout.fillHeight: true
            }
        }

        Rectangle {
            Layout.preferredWidth: 1
            Layout.fillHeight: true
            color: "#2a2a3e"
        }

        RowLayout {
            spacing: 4
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignHCenter

            WorkspaceWidget {
                id: workspaceWidget
                Layout.fillHeight: true
            }
        }

        Rectangle {
            Layout.preferredWidth: 1
            Layout.fillHeight: true
            color: "#2a2a3e"
        }

        RowLayout {
            spacing: 4

            TrayWidget {
                id: trayWidget
                Layout.fillHeight: true
            }

            ClockWidget {
                id: clockWidget
                Layout.fillHeight: true
            }
        }
    }
}
