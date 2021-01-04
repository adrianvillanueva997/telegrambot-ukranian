"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.commandArgsParser = void 0;
var commandArgsParser = function (ctx) {
    if (ctx.message != undefined) {
        var messageContent = ctx.message.text;
        var messageArguments = messageContent.split(' ');
        return {
            command: messageArguments[0],
            arguments: messageArguments.slice(1).map(function (arg) {
                return arg;
            })
        };
    }
    return null;
};
exports.commandArgsParser = commandArgsParser;
