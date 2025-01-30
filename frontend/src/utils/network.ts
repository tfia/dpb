export enum NetworkErrorType {
    NOT_FOUND,
    INVALID_REQUEST,
    INTERNAL_SERVER_ERROR,
    UNKNOWN_ERROR,
}

export class NetworkError extends Error {
    type: NetworkErrorType;
    reason: string;
    message: string;

    constructor(
        _type: NetworkErrorType,
        _reason: string,
        _message: string,
    ) {
        super(_message);

        this.type = _type;
        this.reason = _reason;
        this.message = _message;
    }

    toString(): string { return `${this.reason}: ${this.message}`; }
    valueOf(): string { return `${this.reason}: ${this.message}`; }
}

export const request = async (
    url: string,
    method: "GET" | "POST" | "PUT" | "DELETE",
    body?: object,
) => {
    const response = await fetch(url, {
        method,
        headers: {
            "Content-Type": "application/json",
        },
        body: body && JSON.stringify(body),
    });

    const data = await response.json();
    const code = Number(data.code);
    const reason = data.reason || "UNKNOWN_REASON";
    const message = data.message || "Unknown error occurred.";

    switch (response.status) {
        case 404:
            if (code === 1 && reason === "ERR_NOT_FOUND") {
                throw new NetworkError(NetworkErrorType.NOT_FOUND, reason, message);
            }
            break;
        case 400:
            if (code === 2 && reason === "ERR_INVALID_REQUEST") {
                throw new NetworkError(NetworkErrorType.INVALID_REQUEST, reason, message);
            }
            break;
        case 500:
            if (code === 3 && reason === "ERR_INTERNAL_SERVER_ERROR") {
                throw new NetworkError(NetworkErrorType.INTERNAL_SERVER_ERROR, reason, message);
            }
            break;
    }

    if (!response.ok) {
        throw new NetworkError(NetworkErrorType.UNKNOWN_ERROR, reason, message);
    }

    return data;
};
