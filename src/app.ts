import {Telegraf} from "telegraf";

require('dotenv').config()

import {commandArgsParser} from "./middleware/md_commandsArgument";

const app = new Telegraf(process.env.TelegramBot_Ukranian_Key);

app.command('test', async (ctx) => {
    await ctx.telegram.sendMessage(ctx.message.chat.id, "Patata",
        {reply_to_message_id: ctx.message.message_id}
    ).then()
})

app.command('weather', async (ctx) => {

})


app.launch().then(r => console.log("Bot running!"))