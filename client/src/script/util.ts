const spawn = (f: Function) => setTimeout(f, 0)

const abort = (id: number) => clearTimeout(id)

const sleep = async (ms: number) =>
    await new Promise(resolve => setTimeout(resolve, ms))

const redirect = (url: string) => window.location.href = url
