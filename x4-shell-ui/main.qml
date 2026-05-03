import QtQuick 2.15
import QtQuick.Layouts 1.15
import qs.components 1.0 as Components
import qs.services 1.0 as Services

Item {
    id: root
    
    property var config: Services.ConfigService.config
    property string sidebarPosition: (config && config.sidebar && config.sidebar.position) ? config.sidebar.position : "left"
    
    Loader {
        id: sidebarLoader
        anchors.fill: parent
        sourceComponent: Component {
            Panels.Sidebar {
                anchors.fill: parent
                position: root.sidebarPosition
            }
        }
    }
}
