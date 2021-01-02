import {Telegraf} from "telegraf";
const commandParts = require('telegraf-command-parts');

require('dotenv').config()

const app = new Telegraf(process.env.TelegramBot_Ukranian_Key);
app.use(commandParts)
app.command('test', async (ctx) => {
    console.log(ctx.message)

    await ctx.telegram.sendMessage(ctx.message.chat.id, 'Hello there',
        {reply_to_message_id: ctx.message.message_id}
    ).then()
})

app.command('weather', async (ctx) => {

})


app.launch().then(r => console.log("Bot running!"))