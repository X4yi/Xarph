import QtQuick 2.15
import QtQuick.Layouts 1.15
import qs.components 1.0 as Components
import qs.services 1.0 as Services

Item {
    id: sidebar

    property string position: "left"

    readonly property bool isVertical: position === "left" || position === "right"
    readonly property bool isHorizontal: position === "top" || position === "bottom"

    property color backgroundColor: "#1a1a2e"
    property color borderColor: "#16213e"
    property int borderWidth: 1

    property int verticalWidth: 60
    property int horizontalHeight: 40

    width: isVertical ? verticalWidth : parent.width
    height: isHorizontal ? horizontalHeight : parent.height

    anchors {
        left: position === "left" ? parent.left : undefined
        right: position === "right" ? parent.right : undefined
        top: position === "top" ? parent.top : undefined
        bottom: position === "bottom" ? parent.bottom : undefined
    }

    Rectangle {
        anchors.fill: parent
        color: sidebar.backgroundColor
        border.color: sidebar.borderColor
        border.width: borderWidth

        border.left: position === "right" ? borderWidth : 0
        border.right: position === "left" ? borderWidth : 0
        border.top: position === "bottom" ? borderWidth : 0
        border.bottom: position === "top" ? borderWidth : 0
    }

    Loader {
        id: contentLoader
        anchors.fill: parent
        sourceComponent: sidebar.isVertical ? verticalLayout : horizontalLayout
    }

    Component {
        id: verticalLayout
        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 8
            spacing: 12

            Components.WorkspaceList {
                Layout.alignment: Qt.AlignHCenter
                Layout.fillWidth: true
            }

            Item { Layout.fillHeight: true }

            Components.SystemIndicators {
                Layout.alignment: Qt.AlignHCenter
            }

            Components.Clock {
                Layout.alignment: Qt.AlignHCenter
            }
        }
    }

    Component {
        id: horizontalLayout
        RowLayout {
            anchors.fill: parent
            anchors.margins: 8
            spacing: 12

            Components.WorkspaceList {
                Layout.alignment: Qt.AlignVCenter
                Layout.fillHeight: true
            }

            Item { Layout.fillWidth: true }

            Components.SystemIndicators {
                Layout.alignment: Qt.AlignVCenter
            }

            Components.Clock {
                Layout.alignment: Qt.AlignVCenter
            }
        }
    }
}
