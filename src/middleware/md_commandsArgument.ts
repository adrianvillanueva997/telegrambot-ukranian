import { Context } from "telegraf";

export interface commandData {
  command: string;
  arguments: string[];
}

const commandArgsParser = (ctx: Context) => {
  if (ctx.message != undefined) {
    // @ts-ignore
    // For some reason typescript is idiotic and thinks that the parameter "text"
    // does not exist when it actually exists.
    const messageContent = ctx.message.text;
    console.log(ctx.message);
    const messageArguments = messageContent!.split(" ");
    return {
      command: messageArguments[0],
      arguments: messageArguments.slice(1).map((arg: any) => {
        return arg;
      }),
    } as commandData;
  }
  return null;
};

export { commandArgsParser };
