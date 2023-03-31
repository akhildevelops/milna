export class UserNameUrl {
    name: string
    basepath: string = "/user/"
    constructor(path: URL) {
        console.log(path)
        if (path.pathname.startsWith(this.basepath)) {
            this.name = path.pathname.substring(this.basepath.length)
        } else {
            throw new Error("Cannot create usernameurl")
        }
    }

}

export function createUserNameUrlfromhref(): UserNameUrl {
    return new UserNameUrl(new URL(window.location.href))
}