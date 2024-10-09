$(async () => {
    const server = await server_addr()
    set_title(server + " — Vitium")
    const refresh_chat = async () => {
        try { await recv_chat() }
        catch (e) {
            console.error("chat sync failed: " + e + ", retrying in 10s")
            await sleep(10000)
        }
        if (await chat_modified()) $("#chat-col").html(await render_chat())
    }
    spawn(async () => { while (true) await refresh_chat() })
    $("#form-send-chat").on("submit", async (e) => {
        e.preventDefault()
        const msg = $("#input-msg").val() as string
        $("#input-msg").val("")
        try { await send_chat(msg) }
        catch (e) { console.error("send chat failed: " + e) }
    })
})
