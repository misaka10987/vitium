import React from 'react';
import { createRoot, Root } from 'react-dom/client';

// --- Internal Popup Component ---
// (Component code remains the same as the previous corrected version)
interface PopupComponentProps {
  /** The title text displayed at the top of the popup. */
  title: string;
  /** The main message content of the popup. */
  message: string;
  /** Optional callback function executed when the 'Yes' button is clicked.
   * If provided, 'Yes' and 'No' buttons will be shown. */
  callback_yes?: () => void;
  /** Optional callback function executed when the 'No' button is clicked.
   * Only relevant if `callback_yes` is also provided. */
  callback_no?: () => void;
  /** Internal function called to close and unmount the popup. */
  onClose: () => void;
}

const PopupComponent: React.FC<PopupComponentProps> = ({
  title,
  message,
  callback_yes,
  callback_no,
  onClose,
}) => {
  const showYesNo = typeof callback_yes === 'function';

  const handleYesClick = () => {
    try {
        callback_yes?.();
    } catch (e) {
        console.error("Error in popup 'yes' callback:", e);
    }
    onClose();
  };

  const handleNoClick = () => {
     try {
        callback_no?.();
    } catch (e) {
        console.error("Error in popup 'no' callback:", e);
    }
    onClose();
  };

  const handleOkClick = () => {
    onClose();
  };

  React.useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        if (showYesNo) {
            handleNoClick();
        } else {
            handleOkClick();
        }
      }
    };
    // Ensure document exists before adding listener (relevant for SSR safety, though useEffect helps)
    if(typeof document !== 'undefined') {
        document.addEventListener('keydown', handleKeyDown);
    }
    return () => {
      if(typeof document !== 'undefined') {
          document.removeEventListener('keydown', handleKeyDown);
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [callback_yes, callback_no, onClose, showYesNo]);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-70 p-4 transition-opacity duration-200 ease-out opacity-100">
      <div className="mx-auto w-full max-w-md rounded-lg bg-gray-800 p-6 shadow-xl transform transition-all duration-200 ease-out scale-100 opacity-100">
        <h2 className="text-xl font-semibold text-gray-300 mb-2">{title}</h2>
        <p className="text-gray-300 whitespace-pre-wrap">{message}</p>
        <div className="mt-6 flex justify-end space-x-3">
          {showYesNo ? (
            <>
              <button
                onClick={handleNoClick}
                className="rounded px-4 py-2 text-sm font-medium text-gray-200 bg-gray-700 hover:bg-gray-600 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-800"
                aria-label="No"
              >
                No
              </button>
              <button
                onClick={handleYesClick}
                className="rounded bg-purple-600 px-4 py-2 text-sm font-medium text-white hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 focus:ring-offset-gray-800"
                aria-label="Yes"
              >
                Yes
              </button>
            </>
          ) : (
            <>
              <button
                onClick={handleOkClick}
                className="rounded bg-purple-600 px-4 py-2 text-sm font-medium text-white hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 focus:ring-offset-gray-800"
                aria-label="OK"
              >
                OK
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  );
};


// --- Public Popup Function ---

// Stores the reference to the current popup root if one is active
let popupRoot: Root | null = null;
// Stores the reference to the DOM element where the popup is rendered
let popupContainer: HTMLDivElement | null = null;
// Stores the timeout ID for debouncing rapid calls
let renderTimeoutId: NodeJS.Timeout | null = null;


/**
 * Performs the cleanup of the existing popup (unmounts React component and removes DOM container).
 * Safe to call even if the popup doesn't exist or in non-browser environments.
 */
function cleanupPopup() {
    if (renderTimeoutId) {
        clearTimeout(renderTimeoutId);
        renderTimeoutId = null;
    }

    // Check if we are in a browser environment before trying to unmount/remove
    if (typeof document === 'undefined') {
        return; // Cannot perform cleanup on server
    }

    // Use setTimeout to potentially allow fade-out animation before unmounting
    setTimeout(() => {
        try {
            if (popupRoot) {
                popupRoot.unmount();
            }
        } catch (e) {
            console.error("Error unmounting popup root:", e);
        } finally {
             popupRoot = null; // Ensure it's cleared even if unmount errors
        }

        try {
            if (popupContainer && popupContainer.parentNode) {
                popupContainer.parentNode.removeChild(popupContainer);
            }
        } catch (e) {
            console.error("Error removing popup container:", e);
        } finally {
             popupContainer = null; // Ensure it's cleared
        }
    }, 0); // Defer slightly
}


/**
 * Displays a modal popup in the center of the screen.
 * Only functions in a browser environment. Does nothing during SSR.
 *
 * This function dynamically creates a React root and renders a popup component.
 * It ensures only one popup is visible at a time, replacing any existing one.
 * The rendering is deferred slightly using `setTimeout(..., 0)` to avoid potential
 * conflicts with React's render cycle if called directly from within one.
 *
 * @param title The text to display as the popup's title.
 * @param message The main content/message of the popup. Supports newline characters.
 * @param callback_yes - If provided, the popup will display 'Yes' and 'No' buttons.
 *                      This function will be executed when 'Yes' is clicked, *before* the popup closes.
 * @param callback_no - If `callback_yes` is provided, this function will be executed
 *                     when the 'No' button or Escape key is pressed, *before* the popup closes.
 *                     If `callback_yes` is provided but `callback_no` is not, clicking 'No' or pressing Esc will simply close the popup.
 *                     If `callback_yes` is *not* provided, this parameter is ignored, and an 'OK' button is shown instead (Escape also acts as 'OK').
 *
 * @example
 * // Simple informational popup:
 * // popup("Success", "Your changes have been saved.");
 *
 * @example
 * // Confirmation popup with actions:
 * // popup(
 * //   "Confirm Deletion",
 * //   "Are you sure you want to delete this item?\nThis action cannot be undone.",
 * //   () => { console.log("Item deleted!"); /\* Call delete API *\/ },
 * //   () => { console.log("Deletion cancelled."); }
 * // );
 *
 * @important_usage_note While this function provides a simple imperative API,
 *   for more complex React applications, managing modals declaratively using React state
 *   (e.g., with Context API or a state management library) and `ReactDOM.createPortal`
 *   is generally the recommended and more idiomatic approach. This imperative method
 *   can sometimes lead to edge cases if not used carefully, especially with SSR.
 */
export function popup(
  title: string,
  message: string,
  callback_yes?: () => void,
  callback_no?: () => void
): void {
  // --- Check if running in a browser environment ---
  // If document is not defined (e.g., during SSR), do nothing.
  // This is the crucial check to prevent the error.
  if (typeof document === 'undefined' || typeof window === 'undefined') {
      // Optional: Log a warning during development if called server-side
      if (process.env.NODE_ENV === 'development') {
        console.warn('popup() was called in a non-browser environment (e.g., SSR). Popup cannot be displayed.');
      }
      return; // Exit the function early
  }

  // --- Clean up any existing popup immediately ---
  // Cleanup function now also checks for browser environment internally
  cleanupPopup();

  // --- Defer the creation and rendering ---
  // Use setTimeout to push the execution of this block to the end of the event loop queue.
  renderTimeoutId = setTimeout(() => {
    renderTimeoutId = null; // Clear the timeout ID as it's now executing

    // --- Create container for the new popup ---
    // We already know document exists from the check above.
    try {
        // Protect against edge cases where container might still exist briefly
        if (popupContainer && popupContainer.parentNode) {
            try { popupContainer.parentNode.removeChild(popupContainer); } catch(e){}
            popupContainer = null;
        }

        popupContainer = document.createElement('div');
        popupContainer.id = `dynamic-popup-container-${Date.now()}`;
        popupContainer.className = "fixed inset-0 z-50"; // Base styles
        document.body.appendChild(popupContainer);

        // --- Create a new React root ---
         if (popupRoot) { // Shouldn't happen, but safeguard
             try { popupRoot.unmount(); } catch(e){}
             popupRoot = null;
         }
        popupRoot = createRoot(popupContainer);

        // --- Define the close handler ---
        const handleClose = () => {
            // Call the main cleanup function (which is browser-safe)
            cleanupPopup();
        };

        // --- Render the Popup Component ---
        popupRoot.render(
          <React.StrictMode>
            <PopupComponent
              title={title}
              message={message}
              callback_yes={callback_yes}
              callback_no={callback_no}
              onClose={handleClose}
            />
          </React.StrictMode>
        );
    } catch (error) {
        console.error("Error occurred while creating or rendering the popup:", error);
        // Attempt cleanup in case of partial creation failure
        cleanupPopup();
    }
  }, 0); // Defer execution
}

// Note: Ensure your project's main CSS file includes Tailwind directives.