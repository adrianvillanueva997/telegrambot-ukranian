import {commandData} from './md_commandsArgument'

interface requiredArgs {
    commandData: commandData,
    checking: {
        requiredArgs: number,
        receivedArgs: number,
        checkStatus: boolean
    }
}

export const argsChecker = (commands: commandData, argsNumber: number, minimalArgs : number) => {
    const checking = {
        commandData: commands,
        checking: {
            requiredArgs: argsNumber,
            receivedArgs: commands.arguments.length,
            checkStatus: commands.arguments.length >= minimalArgs
        }
    } as requiredArgs
    if (!checking.checking.checkStatus){
        throw Error(`Expected ${checking.checking.requiredArgs} arguments and got ${checking.checking.receivedArgs}`)
    }
    return checking
}