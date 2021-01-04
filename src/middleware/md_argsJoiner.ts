export const joinArgs = (args: string[]) => {
    let argsAsString : string = ""
    for (let i = 0; i < args.length; i++){
        argsAsString += args[i] + " ";
    }
    return argsAsString;
}