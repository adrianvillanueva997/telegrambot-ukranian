import {Context,} from "telegraf";

export interface commandData {
    command: string,
    arguments: string[]
}

const commandArgsParser = (ctx: Context) => {
    if (ctx.message != undefined) {
        const messageContent = ctx.message.text
        const messageArguments = messageContent!.split(' ')
        return {
            command: messageArguments[0],
            arguments: messageArguments.slice(1).map((arg) => {
                return arg;
            })
        } as commandData
    }
    return null;
}

export {commandArgsParser}