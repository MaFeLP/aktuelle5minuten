/* tslint:disable */
/* eslint-disable */
/**
 * Aktuelle 5 Minuten
 * Aktuelle 5 Minuten API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface Summary200Response
 */
export interface Summary200Response {
    /**
     * The requested category
     * @type {string}
     * @memberof Summary200Response
     */
    category: string;
    /**
     * The text to display to the user
     * @type {string}
     * @memberof Summary200Response
     */
    text: string;
}

/**
 * Check if a given object implements the Summary200Response interface.
 */
export function instanceOfSummary200Response(value: object): boolean {
    if (!('category' in value)) return false;
    if (!('text' in value)) return false;
    return true;
}

export function Summary200ResponseFromJSON(json: any): Summary200Response {
    return Summary200ResponseFromJSONTyped(json, false);
}

export function Summary200ResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): Summary200Response {
    if (json == null) {
        return json;
    }
    return {
        
        'category': json['category'],
        'text': json['text'],
    };
}

export function Summary200ResponseToJSON(value?: Summary200Response | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'category': value['category'],
        'text': value['text'],
    };
}
