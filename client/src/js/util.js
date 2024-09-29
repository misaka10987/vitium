
const spawn = (f) => setTimeout(f, 0);

const sleep = async (ms) =>
    await new Promise(resolve => setTimeout(resolve, ms));
