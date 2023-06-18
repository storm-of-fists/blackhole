import { BaseElement, html, customElement, property, css } from '../base/base.js';
import '@material/web/iconbutton/standard-icon-button.js';
import '@material/web/icon/icon.js';
import '@material/web/linearprogress/linear-progress.js';
import themes from '../../themes/themes.css' assert { type: 'css' };

@customElement('app-home')
export class App extends BaseElement {
  static override styles = [themes, css`
    :host {
      display: block;
    }
  `];

  @property()
  name = 'World';

  @property({type: Number})
  count = 0;

  @property({type: String, reflect: true})
  class = "dark";

  override render() {
    return html`
      <link href="https://fonts.googleapis.com/icon?family=Material+Symbols+Outlined" rel="stylesheet">

      <div>
        <h1>${this.sayHello(this.name)}!</h1>
        <md-standard-icon-button toggle><md-icon>check</md-icon></md-standard-icon-button>
      </div>
      <slot></slot>
      <img src=./assets/test.jpg>
      
    `;
  }

  private _onClick() {
    console.log(this.class)
    this.count++;
    this.dispatchEvent(new CustomEvent('count-changed'));
  }


  sayHello(name: string): string {
    return `Hello, ${name}`;
  }
}
