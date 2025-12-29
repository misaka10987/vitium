import { panic } from "./utils"

export function handleCommand(command: string) {
    const parts = command.trim().split(' ')
    if (parts.length === 0 || parts[0][0] !== '/') {
        panic('Invalid command passed to the command handler, please contact the developer.')
    }
    switch (parts[0].toLowerCase()) {
        case '/help':
            command_help(parts)
            break
        default:
            console.warn(`Unknown command: ${parts[0]}`)
    }
}

function command_help(cmdParts: string[]) {
    
}