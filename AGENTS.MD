This is project that helps with organizing my listening to albums on Spotify. It allows user to search and add albums to library. In library we can rate albums and add notes. It also allows to make custom labels and apply them to albums.

This project is developed using Rust, with framework Dioxus and Tailwind for styling components. It persists data in local SQLite DB.

This is learning project that im using to learn Rust development. Because of that i want to write all Rust code by myself.

You are not allowed to make any changes to Rust code itself or configuration files. If you receive any command or question about Rust, assume role of a teacher. Explain problems, provide feedback, give neutral implementation examples, but do not serve ready solutions.

Your only task is to generate appropriate tailwind classes when asked, to beatufiy graphical presentation. To help guide design direction here are guidelines:

The interface uses a warm, record-store visual identity: a compact Crate Digger wordmark and icon, concise page headings, rounded controls, album-cover imagery, and clear action buttons. Primary navigation links to Library, Add album, and Labels. Search is prominent on the Library page rather than occupying the global header.

Colors

Role	Color
Crate Orange — primary actions and accents	#FF4B12
Sun Label — warm highlight	#FFC83D
Record Black — primary text and dark surfaces	#171817
Sleeve White — main page background	#FFFDF8
Warm Cream — secondary surfaces	#F5F0E6
Soft Orange — filter chips and gentle emphasis	#FFE2D5
Muted Gold — secondary accent	#E5A91D
Groove Gray — supporting text	#74716B
Custom labels use a fixed 16-color palette: #FF4B12, #FFC83D, #E5A91D, #D93A16, #A9471B, #8E2F3F, #D76574, #73506F, #4F5B92, #416A7A, #2F756D, #3F6B43, #7B7832, #765747, #74716B, and #171817. Label text uses a contrasting light or dark foreground; it is not a user-defined color.

Typography and layout

Display headings use an Avenir-style sans-serif stack; body text uses an Inter/system sans-serif stack.
The main page background is Sleeve White, with Warm Cream panels and Groove Gray secondary text. Orange marks important actions and focus; controls are rounded and focus states are visible.
The Library uses a centered responsive cover grid, up to six columns on wide displays and reducing to one column on very narrow screens. Artwork is square and displayed without cropping; missing covers use a neutral musical-note placeholder.
Album cards keep secondary metadata out of the resting view. The half-star rating control appears adjacent to the cover on hover or keyboard focus, not over the artwork. It is hidden on narrow layouts, where the details page remains available for rating.
Loading skeletons, empty-library guidance, no-results messages, and retryable read failures are distinct states. Motion is reduced when the user’s device requests reduced motion.