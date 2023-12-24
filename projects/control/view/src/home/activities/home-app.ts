import { CtLit, html, property, customElement, css } from '@conectate/ct-lit';
import { strings } from '../../xconfig/strings';

@customElement('home-app')
export class HomeApp extends CtLit {
	static styles = [
		css`
			:host {
				display: block;
			}
		`
	];

	render() {
		return html` <h1>${strings.hello_world}</h1>
			<ul>
				<li>
					<a href="/">/ - Home</a>
				</li>

				<li>
					<a href="/login">/login - Login</a>
				</li>

				<li>
					<a href="/404">/404 - NotFound</a>
				</li>
			</ul>`;
	}
}
