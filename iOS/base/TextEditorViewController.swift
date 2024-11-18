import CoreGraphics
import UIKit

class TextEditorViewController : UIViewController {
    override func viewDidLoad() {
        super.viewDidLoad()
        let text = UITextView(frame: CGRect.init(x: 20.0, y: 90.0, width: 250.0, height: 250.0))
        text.center = self.view.center
        text.text = "Hello, World!"
        view.addSubview(text)
    }
}
