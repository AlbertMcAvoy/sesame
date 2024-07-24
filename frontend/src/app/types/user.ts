export interface User {
    id: number,
    mail: string,
    phone: String | null,
    role: Role,
}

export interface Credentials {
    mail: string,
}

enum Role {
    User,
    Client,
    Worker,
    Admin,
}