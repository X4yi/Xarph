import QtQuick 2.15
import QtDBus 1.15

Item {
    id: root
    
    property var daemonInterface: DBusInterface {
        service: "org.x4yi.X4Shell.v1"
        path: "/org/x4yi/X4Shell/v1"
        iface: "org.x4yi.X4Shell.v1"
    }
    
    property var workspaces: []
    property var focusedWorkspace: null
    
    Connections {
        target: root.daemonInterface
        function onWorkspace_changed(workspace) {
            root.onWorkspaceChanged(workspace)
        }
    }
    
    Component.onCompleted: {
        console.log("WorkspaceService initialized")
        if (root.daemonInterface) {
            root.refreshWorkspaces()
        }
    }
    
    function onWorkspaceChanged(workspace) {
        let newWorkspaces = root.workspaces.slice()
        // Update focused state
        for (let i = 0; i < newWorkspaces.length; i++) {
            newWorkspaces[i].focused = (newWorkspaces[i].id === workspace.id)
        }
        root.workspaces = newWorkspaces
        root.focusedWorkspace = workspace
        console.log("Workspace changed:", workspace.name, "focused:", workspace.focused)
    }
    
    function switchWorkspace(workspaceId) {
        if (root.daemonInterface) {
            console.log("Switching to workspace:", workspaceId)
            root.daemonInterface.switchWorkspace(workspaceId)
        }
    }
    
    function refreshWorkspaces() {
        if (root.daemonInterface) {
            var result = root.daemonInterface.getWorkspaces()
            if (result) {
                root.workspaces = result
                console.log("Workspaces refreshed:", result.length)
            }
        }
    }
}
