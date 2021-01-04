import {Telegraf} from "telegraf";
require('dotenv').config();

import {commandArgsParser} from "./middleware/md_commandsArgument";
import {OpenWeather} from "./services/openWeather/sv_OpenWeather";
import {argsChecker} from "./middleware/md_argsChecker";
import {ImageSearch} from "./services/imageSearch/sv_imageSearch";
import {getRandomInt} from "./utilities/randomizer";
import {joinArgs} from "./middleware/md_argsJoiner";


const app = new Telegraf(process.env.TelegramBot_Ukranian_Key!);

app.command('test', async (ctx) => {
    const args = commandArgsParser(ctx);
    if (args != null){
        const joinedArgs = joinArgs(args?.arguments);
        console.log(joinedArgs)
    }
    await ctx.telegram.sendMessage(ctx.message!.chat.id, "Patata",
        {reply_to_message_id: ctx.message!.message_id}
    ).then()
})

app.command('weather', async (ctx) => {
    await ctx.telegram.sendChatAction(ctx.message!.chat.id, "typing")
    const args = commandArgsParser(ctx);
    if (args != null) {
        try {
            argsChecker(args, 1,1);
            const joinedArgs = joinArgs(args.arguments);
            const ow = await new OpenWeather().getWeather(joinedArgs)
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
    await ctx.telegram.sendChatAction(ctx.message!.chat.id, "upload_photo")
    if (args != null){
        try{
            argsChecker(args, 1, 1);
            const joinedArgs = joinArgs(args.arguments);
            const imageSearch = new ImageSearch();
            const images = await imageSearch.getImage(joinedArgs).then((r) => {
                return r as []
            })
            const image = await imageSearch.getRandomImage(images[getRandomInt(0,images.length-1)])
            await ctx.telegram.sendPhoto(ctx.message!.chat.id,image, {caption: image,parse_mode:"HTML"})
        }catch (err){
            await ctx.telegram.sendMessage(ctx.message!.chat.id, err.toString());
        }
    }
})

app.launch().then(r => console.log("Bot running!"))