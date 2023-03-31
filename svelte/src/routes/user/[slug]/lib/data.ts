export interface Link {
    link: string
}
interface ContactI {
    mobile: string
    address: string
}

class Github implements Link {
    link: string
    constructor(link: string) {
        this.link = link
    }
}

class Facebook implements Link {
    link: string
    constructor(link: string) {
        this.link = link
    }
}
class Contact implements ContactI {
    mobile: string
    address: string
    constructor(mobile: string, address: string) {
        this.mobile = mobile
        this.address = address
    }
}

export {
    Github, Facebook, Contact
}

