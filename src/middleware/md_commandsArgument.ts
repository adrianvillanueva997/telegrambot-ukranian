import {Context} from "telegraf";

const commandArgsParser = (ctx: Context) => {
    const messageContent = ctx.message.text
    const messageArguments = messageContent.split(' ')
    return {
        command: messageArguments[0],
        arguments: messageArguments.slice(1).map((arg) => {
            return arg;
        })
    }
}

export {commandArgsParser}