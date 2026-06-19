import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Xarph

Item {
    id: wallpaperLayer

    WallpaperBridge {
        id: wallpaperBridge
        Component.onCompleted: loadConfig()
    }

    // Image wallpaper
    Image {
        id: imageWallpaper
        anchors.fill: parent
        visible: wallpaperBridge.wallpaperType === "image"
        source: wallpaperBridge.wallpaperPath ? "file://" + wallpaperBridge.wallpaperPath : ""
        fillMode: {
            switch (wallpaperBridge.wallpaperMode) {
                case "cover": return Image.PreserveAspectCrop;
                case "contain": return Image.PreserveAspectFit;
                case "stretch": return Image.Stretch;
                case "center": return Image.AlignHCenter | Image.AlignVCenter;
                case "repeat": return Image.Tile;
                default: return Image.PreserveAspectCrop;
            }
        }
    }

    // Color wallpaper
    Rectangle {
        id: colorWallpaper
        anchors.fill: parent
        visible: wallpaperBridge.wallpaperType === "color"
        color: wallpaperBridge.wallpaperColor || "#1a1a2e"
    }

    // Video wallpaper placeholder
    Rectangle {
        id: videoWallpaper
        anchors.fill: parent
        visible: wallpaperBridge.wallpaperType === "video"
        color: "#0a0a12"

        Text {
            anchors.centerIn: parent
            text: "Video Wallpaper"
            color: "#666"
            font.pixelSize: 14
        }
    }

    // Reload wallpaper config every 3 seconds to pick up changes from settings
    Timer {
        interval: 3000
        running: true
        repeat: true
        onTriggered: wallpaperBridge.loadConfig()
    }
}
