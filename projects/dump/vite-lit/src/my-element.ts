import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { MyElement as e1 } from './a1/my-element';

/**
 * An example element.
 */
@customElement('my-element')
export class MyElement extends LitElement {
  /**
   * The number of times the button has been clicked.
   */
  @property({ type: Object })
  count = new e1();

  render() {
    return html`
      <div class="card">
        <button part="button">
          count is ${this.count}
        </button>
      </div>
    `;
  }

  static styles = css``;
}
