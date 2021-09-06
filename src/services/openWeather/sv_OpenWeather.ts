import axios from 'axios';

interface weatherData {
  status: number;
  city: string;
  country: string;
  cord: {
    lon: number;
    lat: number;
  };
  weather: {
    main: string;
    description: string;
  };
  main: {
    temp: number;
    feels_like: number;
    temp_min: number;
    temp_max: number;
    pressure: number;
    humidity: number;
  };
  visibility: number;
  wind: {
    speed: number;
    deg: number;
  };
}

export class OpenWeather {
  private key: string = '';
  private weatherUrl: string =
    'https://api.openweathermap.org/data/2.5/weather';

  constructor() {
    this.getKey();
  }

  private getKey() {
    this.key = process.env.TelegramBot_Ukranian_OpenWeatherKey as string;
  }

  async getWeather(city: string) {
    try {
      return await axios
        .get(`${this.weatherUrl}/?q=${city}&appid=${this.key}&units=metric`)
        .then((r) => {
          return {
            status: r.status,
            city: r.data.name,
            country: r.data.sys.country,
            cord: {
              lon: r.data.coord.lon,
              lat: r.data.coord.lat,
            },
            main: {
              temp: r.data.main.temp,
              feels_like: r.data.main.feels_like,
              temp_min: r.data.main.temp_min,
              temp_max: r.data.main.temp_max,
              pressure: r.data.main.pressure,
              humidity: r.data.main.humidity,
            },
            weather: {
              description: r.data.weather[0].description,
              main: r.data.weather[0].main,
            },
            visibility: r.data.visibility,
            wind: {
              speed: r.data.wind.speed,
              deg: r.data.wind.deg,
            },
          } as weatherData;
        });
    } catch (err: any) {
      return {
        status: err.response.status,
      } as weatherData;
    }
  }
}
