import { BaseElement, html, customElement, property, css } from '../base/base.js';

@customElement('app-home')
export class App extends BaseElement {
  static override styles = css`
    :host {
      display: block;
      border: solid 1px gray;
      padding: 16px;
      max-width: 800px;
    }
  `;

  @property()
  name = 'World';

  @property({type: Number})
  count = 0;

  override render() {
    return html`
      <h1>${this.sayHello(this.name)}!</h1>
      <button @click=${this._onClick} part="button">
        Click Count: ${this.count}
      </button>
      <slot></slot>
      <img src=../../assets/test.jpg>
    `;
  }

  private _onClick() {
    this.count++;
    this.dispatchEvent(new CustomEvent('count-changed'));
  }


  sayHello(name: string): string {
    return `Hello, ${name}`;
  }
}
