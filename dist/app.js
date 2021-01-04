"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
var telegraf_1 = require("telegraf");
require('dotenv').config();
var md_commandsArgument_1 = require("./middleware/md_commandsArgument");
var sv_OpenWeather_1 = require("./services/openWeather/sv_OpenWeather");
var md_argsChecker_1 = require("./middleware/md_argsChecker");
var sv_imageSearch_1 = require("./services/imageSearch/sv_imageSearch");
var randomizer_1 = require("./utilities/randomizer");
var md_argsJoiner_1 = require("./middleware/md_argsJoiner");
var sv_4chan_1 = require("./services/4chan/sv_4chan");
var app = new telegraf_1.Telegraf(process.env.TelegramBot_Ukranian_Key);
var sendErrorMessage = function (ctx, err) { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0: return [4 /*yield*/, ctx.telegram.sendMessage(ctx.message.chat.id, err.toString())];
            case 1:
                _a.sent();
                return [2 /*return*/];
        }
    });
}); };
app.command('test', function (ctx) { return __awaiter(void 0, void 0, void 0, function () {
    var args, joinedArgs;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                args = md_commandsArgument_1.commandArgsParser(ctx);
                if (args != null) {
                    joinedArgs = md_argsJoiner_1.joinArgs(args === null || args === void 0 ? void 0 : args.arguments);
                    console.log(joinedArgs);
                }
                return [4 /*yield*/, ctx.telegram.sendMessage(ctx.message.chat.id, "Patata", { reply_to_message_id: ctx.message.message_id }).then()];
            case 1:
                _a.sent();
                return [2 /*return*/];
        }
    });
}); });
app.command('weather', function (ctx) { return __awaiter(void 0, void 0, void 0, function () {
    var args, joinedArgs, ow, err_1;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0: return [4 /*yield*/, ctx.telegram.sendChatAction(ctx.message.chat.id, "typing")];
            case 1:
                _a.sent();
                args = md_commandsArgument_1.commandArgsParser(ctx);
                if (!(args != null)) return [3 /*break*/, 10];
                _a.label = 2;
            case 2:
                _a.trys.push([2, 8, , 10]);
                md_argsChecker_1.argsChecker(args, 1, 1);
                joinedArgs = md_argsJoiner_1.joinArgs(args.arguments);
                return [4 /*yield*/, new sv_OpenWeather_1.OpenWeather().getWeather(joinedArgs)];
            case 3:
                ow = _a.sent();
                if (!(ow.status === 404)) return [3 /*break*/, 5];
                return [4 /*yield*/, ctx.telegram.sendMessage(ctx.message.chat.id, "City \"" + args.arguments[0] + "\" not found!")];
            case 4:
                _a.sent();
                return [3 /*break*/, 7];
            case 5:
                if (!(ow.status === 200)) return [3 /*break*/, 7];
                return [4 /*yield*/, ctx.telegram.sendMessage(ctx.message.chat.id, ("<b>" + ow.city + "," + ow.country + " (" + ow.cord.lon + "," + ow.cord.lat + ")</b>\n\n<b>Current Temperature:</b> " + ow.main.temp + "\u00BAC\n<b>Current weather:</b> " + ow.weather.main + ", " + ow.weather.description + "\n<b>Max Temperature:</b> " + ow.main.temp_max + "\u00BAC\n<b>Min Temperature:</b> " + ow.main.temp_min + "\u00BAC\n<b>Temperature feels like:</b> " + ow.main.feels_like + "\u00BAC\n<b>Wind:</b> " + ow.wind.speed + "m/s, " + ow.wind.deg + "\u00BA\n<b>Pressure:</b> " + ow.main.pressure + "hPa\n<b>Humidty:</b> " + ow.main.humidity + "%\n<b>Visibility:</b>" + ow.visibility + "m\n"), { parse_mode: "HTML" })];
            case 6:
                _a.sent();
                _a.label = 7;
            case 7: return [3 /*break*/, 10];
            case 8:
                err_1 = _a.sent();
                return [4 /*yield*/, sendErrorMessage(ctx, err_1)];
            case 9:
                _a.sent();
                return [3 /*break*/, 10];
            case 10: return [2 /*return*/];
        }
    });
}); });
app.command('get', function (ctx) { return __awaiter(void 0, void 0, void 0, function () {
    var args, joinedArgs, imageSearch, images, image, imageStatus, err_2;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                args = md_commandsArgument_1.commandArgsParser(ctx);
                return [4 /*yield*/, ctx.telegram.sendChatAction(ctx.message.chat.id, "upload_photo")];
            case 1:
                _a.sent();
                if (!(args != null)) return [3 /*break*/, 11];
                _a.label = 2;
            case 2:
                _a.trys.push([2, 9, , 11]);
                md_argsChecker_1.argsChecker(args, 1, 1);
                joinedArgs = md_argsJoiner_1.joinArgs(args.arguments);
                imageSearch = new sv_imageSearch_1.ImageSearch();
                return [4 /*yield*/, imageSearch.getImage(joinedArgs).then(function (r) {
                        return r;
                    })];
            case 3:
                images = _a.sent();
                return [4 /*yield*/, imageSearch.getRandomImage(images[randomizer_1.getRandomInt(0, images.length - 1)])];
            case 4:
                image = _a.sent();
                return [4 /*yield*/, imageSearch.checkImageStatus(image)];
            case 5:
                imageStatus = _a.sent();
                if (!(imageStatus != 200 || imageStatus == null)) return [3 /*break*/, 7];
                return [4 /*yield*/, imageSearch.getRandomImage(images[randomizer_1.getRandomInt(0, images.length - 1)])];
            case 6:
                image = _a.sent();
                _a.label = 7;
            case 7: return [4 /*yield*/, ctx.telegram.sendPhoto(ctx.message.chat.id, image, { caption: image, parse_mode: "HTML" })];
            case 8:
                _a.sent();
                return [3 /*break*/, 11];
            case 9:
                err_2 = _a.sent();
                return [4 /*yield*/, sendErrorMessage(ctx, err_2)];
            case 10:
                _a.sent();
                return [3 /*break*/, 11];
            case 11: return [2 /*return*/];
        }
    });
}); });
app.command('getboards', function (ctx) { return __awaiter(void 0, void 0, void 0, function () {
    var chan, boards, message, i, board, title, err_3;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                _a.trys.push([0, 4, , 6]);
                return [4 /*yield*/, ctx.telegram.sendChatAction(ctx.message.chat.id, "typing")];
            case 1:
                _a.sent();
                chan = new sv_4chan_1.Chan();
                return [4 /*yield*/, chan.getBoards()];
            case 2:
                boards = _a.sent();
                message = "";
                for (i = 0; i < boards.length; i++) {
                    board = boards[i].board;
                    title = boards[i].title;
                    message += "<b>" + board + "</b>: " + title + "\n";
                }
                return [4 /*yield*/, ctx.telegram.sendMessage(ctx.message.chat.id, message, { parse_mode: "HTML" })];
            case 3:
                _a.sent();
                return [3 /*break*/, 6];
            case 4:
                err_3 = _a.sent();
                return [4 /*yield*/, sendErrorMessage(ctx, err_3)];
            case 5:
                _a.sent();
                return [3 /*break*/, 6];
            case 6: return [2 /*return*/];
        }
    });
}); });
app.command('getfunnyshit', function (ctx) { return __awaiter(void 0, void 0, void 0, function () {
    var chan, webm, err_4;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                _a.trys.push([0, 4, , 6]);
                return [4 /*yield*/, ctx.telegram.sendChatAction(ctx.message.chat.id, "record_video")];
            case 1:
                _a.sent();
                chan = new sv_4chan_1.Chan();
                return [4 /*yield*/, chan.getRandomWsgWebm()];
            case 2:
                webm = _a.sent();
                return [4 /*yield*/, ctx.telegram.sendMessage(ctx.message.chat.id, "<a href=\"" + webm.fileURL + "\">" + webm.fileTitle + "</a>", { parse_mode: "HTML" })];
            case 3:
                _a.sent();
                return [3 /*break*/, 6];
            case 4:
                err_4 = _a.sent();
                return [4 /*yield*/, sendErrorMessage(ctx, err_4)];
            case 5:
                _a.sent();
                return [3 /*break*/, 6];
            case 6: return [2 /*return*/];
        }
    });
}); });
app.command('call', function (ctx) { return __awaiter(void 0, void 0, void 0, function () {
    var imageSearch, images, image, imageStatus, err_5;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                _a.trys.push([0, 7, , 9]);
                imageSearch = new sv_imageSearch_1.ImageSearch();
                return [4 /*yield*/, imageSearch.getImage("dota 2 memes").then(function (r) {
                        return r;
                    })];
            case 1:
                images = _a.sent();
                return [4 /*yield*/, imageSearch.getRandomImage(images[randomizer_1.getRandomInt(0, images.length - 1)])];
            case 2:
                image = _a.sent();
                return [4 /*yield*/, imageSearch.checkImageStatus(image)];
            case 3:
                imageStatus = _a.sent();
                if (!(imageStatus != 200 || imageStatus == null)) return [3 /*break*/, 5];
                return [4 /*yield*/, imageSearch.getRandomImage(images[randomizer_1.getRandomInt(0, images.length - 1)])];
            case 4:
                image = _a.sent();
                _a.label = 5;
            case 5: return [4 /*yield*/, ctx.telegram.sendPhoto(ctx.message.chat.id, image, { caption: "(Doto) @thexiao77, @lilnarwhal, @dvdgg, @SanZ97xX, @dark_trainer" })];
            case 6:
                _a.sent();
                return [3 /*break*/, 9];
            case 7:
                err_5 = _a.sent();
                return [4 /*yield*/, sendErrorMessage(ctx, err_5)];
            case 8:
                _a.sent();
                return [3 /*break*/, 9];
            case 9: return [2 /*return*/];
        }
    });
}); });
app.command('cs', function (ctx) { return __awaiter(void 0, void 0, void 0, function () {
    var imageSearch, images, image, imageStatus, err_6;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                _a.trys.push([0, 7, , 9]);
                imageSearch = new sv_imageSearch_1.ImageSearch();
                return [4 /*yield*/, imageSearch.getImage("csgo memes").then(function (r) {
                        return r;
                    })];
            case 1:
                images = _a.sent();
                return [4 /*yield*/, imageSearch.getRandomImage(images[randomizer_1.getRandomInt(0, images.length - 1)])];
            case 2:
                image = _a.sent();
                return [4 /*yield*/, imageSearch.checkImageStatus(image)];
            case 3:
                imageStatus = _a.sent();
                if (!(imageStatus != 200 || imageStatus == null)) return [3 /*break*/, 5];
                return [4 /*yield*/, imageSearch.getRandomImage(images[randomizer_1.getRandomInt(0, images.length - 1)])];
            case 4:
                image = _a.sent();
                _a.label = 5;
            case 5: return [4 /*yield*/, ctx.telegram.sendPhoto(ctx.message.chat.id, image, {
                    caption: "(CSGO) @thexiao77, @lilnarwhal, @joseawe, @DavasJoe " +
                        ",@dark_trainer, @Sauturn, @REDMSR, @txc450, @THEDRDVD",
                })];
            case 6:
                _a.sent();
                return [3 /*break*/, 9];
            case 7:
                err_6 = _a.sent();
                return [4 /*yield*/, sendErrorMessage(ctx, err_6)];
            case 8:
                _a.sent();
                return [3 /*break*/, 9];
            case 9: return [2 /*return*/];
        }
    });
}); });
app.launch().then(function (r) { return console.log("Bot running!"); });
