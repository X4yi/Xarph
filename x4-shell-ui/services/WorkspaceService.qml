import QtQuick 2.15
import QtDBus 1.15

Item {
    id: root

    property var daemonInterface: DBusInterface {
        service: "org.x4yi.X4Shell"
        path: "/org/x4yi/X4Shell/v1"
        iface: "org.x4yi.X4Shell.v1"
    }

    property var workspaces: []
    property var focusedWorkspace: null

    Component.onCompleted: {
        console.log("WorkspaceService initialized")
        if (root.daemonInterface) {
            root.daemonInterface.onWorkspaceChanged.connect(root.onWorkspaceChanged)
            root.refreshWorkspaces()
        }
    }

    function onWorkspaceChanged(workspace) {
        let newWorkspaces = root.workspaces.slice()
        root.workspaces = newWorkspaces
        root.focusedWorkspace = workspace
    }

    function switchWorkspace(workspaceId) {
        if (root.daemonInterface) {
            root.daemonInterface.switchWorkspace(workspaceId)
        }
    }

    function refreshWorkspaces() {
        if (root.daemonInterface) {
            var result = root.daemonInterface.getWorkspaces()
            if (result) {
                root.workspaces = result
            }
        }
    }
}
