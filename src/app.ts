import {Telegraf} from "telegraf";
require('dotenv').config();
const emoji = require('node-emoji');

import {commandArgsParser} from "./middleware/md_commandsArgument";
import {OpenWeather} from "./services/openWeather/sv_OpenWeather";
import {argsChecker} from "./middleware/md_argsChecker";
import {ImageSearch} from "./services/imageSearch/sv_imageSearch";


const app = new Telegraf(process.env.TelegramBot_Ukranian_Key!);

app.command('test', async (ctx) => {
    const args = commandArgsParser(ctx);
    console.log(args)
    await ctx.telegram.sendMessage(ctx.message!.chat.id, "Patata",
        {reply_to_message_id: ctx.message!.message_id}
    ).then()
})

app.command('weather', async (ctx) => {
    const args = commandArgsParser(ctx);
    if (args != null) {
        try {
            argsChecker(args, 1);
            const ow = await new OpenWeather().getWeather(args.arguments[0])
            if (ow.status === 404) {
                await ctx.telegram.sendMessage(ctx.message!.chat.id, `City "${args.arguments[0]}" not found!`)
            } else if (ow.status === 200) {
                await ctx.telegram.sendMessage(ctx.message!.chat.id, (`<b>${ow.city},${ow.country} (${ow.cord.lon},${ow.cord.lat})</b>\n
<u>Current Temperature:</u> ${ow.main.temp}ºC
<u>Current weather:</u> ${ow.weather.main},${ow.weather.description}
<u>Max Temperature:</u> ${ow.main.temp_max}ºC
<u>Min Temperature:</u> ${ow.main.temp_min}ºC
<u>Temperature feels like:</u> ${ow.main.feels_like}ºC
<u>Wind:</u> ${ow.wind.speed}m/s, ${ow.wind.deg}º
<u>Pressure:</u> ${ow.main.pressure}hPa
<u>Humidty:</u> ${ow.main.humidity}%
<u>Visibility:</u>${ow.visibility}m
`), {parse_mode: "HTML"});
            }
        } catch (err) {
            await ctx.telegram.sendMessage(ctx.message!.chat.id, err.toString())
        }
    }
})

app.command('get', async (ctx) => {
    const args = commandArgsParser(ctx);
    if (args != null){
        try{
            argsChecker(args, 1);
            const imageSearch = new ImageSearch();
            await imageSearch.getImage(args.arguments[0])
        }catch (err){
            await ctx.telegram.sendMessage(ctx.message!.chat.id, err.toString());
        }
    }
})

app.launch().then(r => console.log("Bot running!"))