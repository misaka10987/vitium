$(() => {
    $("#form-connect").on("submit", async (e) => {
        e.preventDefault()
        const server = $("#input-server").val() as string
        const user = $("#input-user").val() as string
        const pass = $("#input-pass").val() as string
        try {
            await login(server, user, pass)
            redirect("/ui/hall")
        }
        catch (err) {
            $("#err-msg-content").html(err as string)
            $("#err-msg").show()
        }
    })
})
